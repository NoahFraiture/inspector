<script>
	export let left = 100;
	export let top = 100;

	let moving = false;

	function onMouseDown() {
		moving = true;
	}

	function onMouseMove(e) {
		if (moving) {
			// NOTE: works only on linux, on windows and macos it's probably a delta
			// this it should be 'left += e.movementX'
			left = e.movementX;
			top = e.movementY;
		}
	}

	function onMouseUp() {
		moving = false;
	}

	// 	$: console.log(moving);
</script>

<section on:mousedown={onMouseDown} style="left: {left}px; top: {top}px;" class="draggable">
	<slot></slot>
</section>

<svelte:window on:mouseup={onMouseUp} on:mousemove={onMouseMove} />

<style>
	.draggable {
		user-select: none;
		cursor: move;
		border: solid 1px gray;
		position: absolute;
	}
</style>
