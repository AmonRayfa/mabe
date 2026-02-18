import {execSync} from "node:child_process"

try {
  console.log("🗑️ Cleaning cached API reference...")
  execSync("rm -rf target/doc docs/gen")

  console.log("📚 Building the API reference...")
  // Inherit stdio so we see the output colors
  execSync("cargo doc --workspace --no-deps", {stdio: "inherit"})

  console.log("📦 Moving the API reference to ./docs...")
  execSync("cp -r target/doc docs/gen")

  console.log("✅ Done!")
} catch (e) {
  console.error("🚫 Build failed: ", e.message)
  process.exit(1)
}
