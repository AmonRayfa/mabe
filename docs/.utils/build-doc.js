import {execSync} from "node:child_process"

try {
  console.log("🗑️ Cleaning cached documentation...")
  execSync("rm -rf target/doc docs/dist")

  console.log("📚 Building the documentation...")
  // Inherit stdio so we see the output colors
  execSync("cargo doc --workspace --no-deps", {stdio: "inherit"})

  console.log("📦 Moving the documentation to ./docs/dist...")
  execSync("cp -r target/doc docs/dist")

  console.log("✅ Done!")
} catch (e) {
  console.error("🚫 Build failed: ", e.message)
  process.exit(1)
}
