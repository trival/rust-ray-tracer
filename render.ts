import { $ } from "bun"
import * as fs from "node:fs/promises"
import { parseArgs } from "node:util"

const { positionals, values } = parseArgs({
	args: Bun.argv.slice(2),
	options: {
		count: {
			type: "string",
			short: "n",
			default: "1",
		},
		help: {
			type: "boolean",
			short: "h",
		},
		timestamp: {
			type: "boolean",
			short: "t",
		},
	},
	allowPositionals: true,
	strict: true,
})

if (values.help) {
	printHelp()
}

if (positionals.length !== 1) {
	console.error("Expected exactly one example name argument")
	printHelp(1)
}

const count = parseInt(values.count!)

if (Number.isNaN(count) || count <= 0) {
	console.error("Count must be a positive integer")
	printHelp(1)
}

const exampleName = positionals[0]

await fs.mkdir(`out/${exampleName}`, { recursive: true })
const currentTime = new Date().toISOString().split(".")[0].replace(/:/g, "-")

function padNumber(num: number, length: number) {
	return num.toString().padStart(length, "0")
}

function fileName(i?: number) {
	if (i) {
		if (values.timestamp) {
			return `out/${exampleName}/${currentTime}-${padNumber(i, 3)}.ppm`
		} else {
			return `out/${exampleName}/${padNumber(i, 3)}.ppm`
		}
	} else {
		if (values.timestamp) {
			return `out/${exampleName}/${currentTime}.ppm`
		} else {
			return `out/${exampleName}/output.ppm`
		}
	}
}

console.time("Render total")
if (count > 1) {
	for (let i = 1; i <= count; i++) {
		console.log(`Rendering ${i}/${count}`)
		console.time(`Execution time ${i}`)
		await $`time cargo run --release --example ${exampleName} > ${fileName(i)}`
		console.timeEnd(`Execution time ${i}`)
		console.log("")
	}
} else {
	await $`time cargo run --release --example ${exampleName} > ${fileName()}`
}
console.timeEnd("Render total")

function printHelp(exitCode = 0) {
	console.log("")
	console.log("Usage: bun render.ts [options] <example name>")
	console.log("Options:")
	console.log("  -n, --count <n>  Number of times to run the example")
	console.log("  -t, --timestamp  Include a timestamp in the output file name")
	console.log("  -h, --help       Print this help message")
	process.exit(exitCode)
}
