<script>
	export let views = [];
	export let changeView = () => {};
	let activeView = views[0].id;

	function updateView(viewId) {
		activeView = viewId;
		changeView(viewId);
	}
</script>

<div class="sidebar">
	{#each views as view}
		<button class:active={activeView === view.id} on:click={() => updateView(view.id)}>
			{view.name}
		</button>
	{/each}
</div>

<style>
	:root {
		--sidebar-bg-color: #343a40;
		--sidebar-text-color: #ffffff;
		--sidebar-hover-bg-color: #495057;
		--sidebar-active-color: #17a2b8;
	}

	.sidebar {
		width: 200px;
		height: 100vh;
		background-color: var(--sidebar-bg-color);
		color: var(--sidebar-text-color);
		display: flex;
		flex-direction: column;
		padding: 10px;
	}

	button {
		background: none;
		border: none;
		color: inherit;
		padding: 10px;
		margin: 5px 0;
		width: 100%;
		text-align: left;
		font-size: 16px;
		transition: background-color 0.3s;
	}

	button:hover {
		background-color: var(--sidebar-hover-bg-color);
		cursor: pointer;
	}

	.active {
		background-color: var(--sidebar-active-color);
	}
</style>
