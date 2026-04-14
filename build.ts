import {NapiCli} from "@napi-rs/cli";

const cli = new NapiCli()

// cli.createNpmDirs({packageJsonPath:"package/package.json",npmDir:"package/npm"})
// cli.artifacts({npmDir: "package/npm",packageJsonPath:"package/package.json",buildOutputDir:"package/npm",configPath:""})
cli.universalize({
  packageJsonPath: "package/package.json",
  outputDir: "package",
})
// cli.build({
//   configPath: "package/package.json",
//   packageJsonPath: "package/package.json",
//   esm: true,
//   constEnum: false,
//   outputDir: "package",
//   jsBinding: "index.js",
//   platform: true,
//   useNapiCross: true,
// })