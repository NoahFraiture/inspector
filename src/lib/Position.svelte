<script>
	import { invoke } from '@tauri-apps/api/tauri';

	let position = [0, 0];

	async function update() {
		position = await invoke('mouse_position', {});
	}
</script>

<div>
	<button on:click={update}>Update</button>
	<p>{position}</p>
</div>
