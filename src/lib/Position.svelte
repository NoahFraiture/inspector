<script>
	import { invoke } from '@tauri-apps/api/tauri';

	let x = 0;
	let y = 0;
	let number = 0;

	async function update() {
		let tuple = await invoke('update', {});
		x = tuple[0];
		y = tuple[1];
	}
</script>

<div>
	<button on:click={update}>Update</button>
	<p>{x} {y}</p>
	<p>{number}</p>
</div>
