// @ts-check
"use strict";

const path = require("path");
const fs = require("fs");
const os = require("os");
const https = require("https");
const util = require("util");
const url = require("url");
const URL = url.URL;
const child_process = require("child_process");
const proxy_from_env = require("proxy-from-env");

const get = require("./get");

const packageVersion = require("../package.json").version;
const tmpDir = path.join(os.tmpdir(), `easywind-cache-${packageVersion}`);

const fsUnlink = util.promisify(fs.unlink);
const fsExists = util.promisify(fs.exists);
const fsMkdir = util.promisify(fs.mkdir);

const isWindows = os.platform() === "win32";

/**
 * @param {string} _url
 * @returns boolean
 */
function isGithubUrl(_url) {
  return url.parse(_url).hostname === "api.github.com";
}

/**
 * @param {string} url
 * @param {string} dest
 * @param {{ headers: Record<string, string>; proxy?: string; }} opts
 * @returns boolean
 */
function downloadWin(url, dest, opts) {
  return new Promise((resolve, reject) => {
    let userAgent;
    if (opts.headers["user-agent"]) {
      userAgent = opts.headers["user-agent"];
      delete opts.headers["user-agent"];
    }
    const headerValues = Object.keys(opts.headers)
      .map((key) => `\\"${key}\\"=\\"${opts.headers[key]}\\"`)
      .join("; ");
    const headers = `@{${headerValues}}`;
    console.log("Downloading with Invoke-WebRequest");
    dest = sanitizePathForPowershell(dest);
    let iwrCmd = `[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; Invoke-WebRequest -URI ${url} -UseBasicParsing -OutFile ${dest} -Headers ${headers}`;
    if (userAgent) {
      iwrCmd += " -UserAgent " + userAgent;
    }
    if (opts.proxy) {
      iwrCmd += " -Proxy " + opts.proxy;

      try {
        const { username, password } = new URL(opts.proxy);
        if (username && password) {
          const decodedPassword = decodeURIComponent(password);
          iwrCmd += ` -ProxyCredential (New-Object PSCredential ('${username}', (ConvertTo-SecureString '${decodedPassword}' -AsPlainText -Force)))`;
        }
      } catch (err) {
        reject(err);
      }
    }

    iwrCmd = `powershell "${iwrCmd}"`;

    child_process.exec(iwrCmd, (err) => {
      if (err) {
        reject(err);
        return;
      }
      resolve();
    });
  });
}

function download(_url, dest, opts) {
  const proxy = proxy_from_env.getProxyForUrl(url.parse(_url));
  if (proxy !== "") {
    var HttpsProxyAgent = require("https-proxy-agent");
    opts = {
      ...opts,
      agent: new HttpsProxyAgent(proxy),
      proxy,
    };
  }

  if (isWindows) {
    // This alternative strategy shouldn't be necessary but sometimes on Windows the file does not get closed,
    // so unzipping it fails, and I don't know why.
    return downloadWin(_url, dest, opts);
  }

  if (opts.headers.authorization && !isGithubUrl(_url)) {
    delete opts.headers.authorization;
  }

  return new Promise((resolve, reject) => {
    console.log(`Download options: ${JSON.stringify(opts)}`);
    const outFile = fs.createWriteStream(dest);
    const mergedOpts = {
      ...url.parse(_url),
      ...opts,
    };
    https
      .get(mergedOpts, (response) => {
        console.log("statusCode: " + response.statusCode);
        if (response.statusCode === 302) {
          console.log("Following redirect to: " + response.headers.location);
          return download(response.headers.location, dest, opts).then(
            resolve,
            reject
          );
        } else if (response.statusCode !== 200) {
          reject(new Error("Download failed with " + response.statusCode));
          return;
        }

        response.pipe(outFile);
        outFile.on("finish", () => {
          resolve();
        });
      })
      .on("error", async (err) => {
        await fsUnlink(dest);
        reject(err);
      });
  });
}

/**
 * @param {{ force: boolean; token: string; version: string; }} opts
 * @param {string} assetName
 * @param {string} downloadFolder
 * @return {Promise<void>}
 */
async function getAssetFromGitHub(opts, assetName, downloadFolder) {
  const assetDownloadPath = path.join(downloadFolder, assetName);

  // We can just use the cached binary
  if (!opts.force && (await fsExists(assetDownloadPath))) {
    console.log("Using cached download: " + assetDownloadPath);
    return;
  }

  const downloadOpts = {
    headers: {
      "user-agent": "easywind",
    },
  };

  downloadOpts.headers.accept = "application/octet-stream";
  if (opts.token) {
    downloadOpts.headers.authorization = `token ${opts.token}`;
  }

  const jsonRelease = require("../release.json");
  const asset = jsonRelease.assets.find((a) => a.name === assetName);
  if (!asset) {
    throw new Error("Asset not found with name: " + assetName);
  }

  console.log(`Downloading from ${asset.url}`);
  console.log(`Downloading to ${assetDownloadPath}`);

  try {
    await download(asset.url, assetDownloadPath, downloadOpts);
  } catch (e) {
    console.error("Download failed:", e);
    console.error(
      `Attempting to download from 'browser download url' ${asset.browser_download_url} instead`
    );

    delete downloadOpts.headers.authorization;
    await download(asset.browser_download_url, assetDownloadPath, downloadOpts);
  }
}

function unzipWindows(zipPath, destinationDir) {
  return new Promise((resolve, reject) => {
    zipPath = sanitizePathForPowershell(zipPath);
    destinationDir = sanitizePathForPowershell(destinationDir);
    const expandCmd =
      "powershell -ExecutionPolicy Bypass -Command Expand-Archive " +
      ["-Path", zipPath, "-DestinationPath", destinationDir, "-Force"].join(
        " "
      );
    child_process.exec(expandCmd, (err, _stdout, stderr) => {
      if (err) {
        reject(err);
        return;
      }

      if (stderr) {
        console.log(stderr);
        reject(new Error(stderr));
        return;
      }

      console.log("Expand-Archive completed");
      resolve();
    });
  });
}

// Handle whitespace in filepath as powershell split's path with whitespaces
function sanitizePathForPowershell(path) {
  path = path.replace(/ /g, "` "); // replace whitespace with "` " as solution provided here https://stackoverflow.com/a/18537344/7374562
  return path;
}

function untar(zipPath, destinationDir) {
  return new Promise((resolve, reject) => {
    const unzipProc = child_process.spawn(
      "tar",
      ["xvf", zipPath, "-C", destinationDir],
      { stdio: "inherit" }
    );
    unzipProc.on("error", (err) => {
      reject(err);
    });
    unzipProc.on("close", (code) => {
      console.log(`tar xvf exited with ${code}`);
      if (code !== 0) {
        reject(new Error(`tar xvf exited with ${code}`));
        return;
      }

      resolve();
    });
  });
}

async function unzipEasywind(zipPath, destinationDir) {
  if (isWindows) {
    await unzipWindows(zipPath, destinationDir);
  } else {
    await untar(zipPath, destinationDir);
  }

  const expectedName = path.join(destinationDir, "easywind");
  if (await fsExists(expectedName)) {
    return expectedName;
  }

  if (await fsExists(expectedName + ".exe")) {
    return expectedName + ".exe";
  }

  throw new Error(
    `Expecting easywind or easywind.exe unzipped into ${destinationDir}, didn't find one.`
  );
}

module.exports = async (opts) => {
  if (!opts.version) {
    return Promise.reject(new Error("Missing version"));
  }

  if (!opts.target) {
    return Promise.reject(new Error("Missing target"));
  }

  const extension = isWindows ? ".zip" : ".tar.gz";
  const assetName =
    ["easywind", opts.version, opts.target].join("-") + extension;

  if (!(await fsExists(tmpDir))) {
    await fsMkdir(tmpDir);
  }

  const assetDownloadPath = path.join(tmpDir, assetName);
  try {
    await getAssetFromGitHub(opts, assetName, tmpDir);
  } catch (e) {
    console.log("Deleting invalid download cache");
    try {
      await fsUnlink(assetDownloadPath);
    } catch (e) { }

    throw e;
  }

  console.log(`Unzipping to ${opts.destDir}`);
  try {
    const destinationPath = await unzipEasywind(
      assetDownloadPath,
      opts.destDir
    );
    if (!isWindows) {
      await util.promisify(fs.chmod)(destinationPath, "755");
    }
  } catch (e) {
    console.log("Deleting invalid download");

    try {
      await fsUnlink(assetDownloadPath);
    } catch (e) { }

    throw e;
  }
};
