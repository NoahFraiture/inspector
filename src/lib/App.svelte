<script>
	import Sidebar from './Sidebar.svelte';
	import ViewA from './ViewA.svelte';

	let views = [
		{ id: 'A', name: 'View A' },
		{ id: 'B', name: 'View B' },
		{ id: 'C', name: 'View C' }
	];

	let currentView = 'A'; // Default view

	// Function to change view
	function changeView(view) {
		currentView = view;
	}
</script>

<div style="display: flex;">
	<Sidebar {views} {changeView} />
	<div style="flex-grow: 1; padding: 20px;">
		{#if currentView === 'A'}
			<ViewA />
		{:else if currentView === 'B'}
			<ViewA />
		{:else if currentView === 'C'}
			<ViewA />
		{/if}
	</div>
</div>
