export function read_file() {
		try {
			return Uint8Array.from(require('!!raw-loader!./../../ukulele-a-440.wav'));
		} catch (err) {
			console.log(err);
		}
}
