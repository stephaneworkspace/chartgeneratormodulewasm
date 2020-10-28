export function read_file() {
		console.log('2');
		try {
			const SAMPLE = Uint8Array.from(require('!!raw-loader!./ukulele-a-440.bin'));
			console.log(SAMPLE);
		} catch (err) {
			console.log(err);
		}
		return "ok";
}
