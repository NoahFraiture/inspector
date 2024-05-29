<script lang="ts">
	import { graphNameStore, graphName } from '../store';
	import { onMount, onDestroy } from 'svelte';
	let dropdownOpen = false;
	let buttonElement: HTMLButtonElement;

	// Function to toggle dropdown visibility
	function toggleDropdown() {
		dropdownOpen = !dropdownOpen;
	}

	// Function to close the dropdown when clicking outside of it
	function handleClickOutside(event: MouseEvent) {
		if (!buttonElement.contains(event.target as Node)) {
			dropdownOpen = false;
		}
	}

	function selectOption(option: string) {
		graphNameStore.set(option);
	}

	onMount(() => {
		document.addEventListener('click', handleClickOutside);
	});

	onDestroy(() => {
		document.removeEventListener('click', handleClickOutside);
	});
</script>

<div>
	<button class="dropdown-btn" bind:this={buttonElement} on:click={toggleDropdown}>
		{$graphName}▼
	</button>
	<div class="dropdown-content {dropdownOpen ? 'show' : ''}">
		<a href="#" class="dropdown-link" on:click={() => selectOption('Option 1')}>Option 1</a>
		<a href="#" class="dropdown-link" on:click={() => selectOption('Option 2')}>Option 2</a>
		<a href="#" class="dropdown-link" on:click={() => selectOption('Option 3')}>Option 3</a>
		<!-- Add as many options as needed -->
	</div>
</div>

<style>
	.dropdown-content {
		display: none;
		position: absolute;
		background-color: #f1f1f1;
		min-width: 160px;
		box-shadow: 0 8px 16px rgba(0, 0, 0, 0.2);
		z-index: 1;
		max-height: 200px;
		overflow-y: auto;
		border-radius: 5px;
		margin-top: 5px;
	}

	.dropdown-content.show {
		display: block;
	}

	.dropdown-btn {
		background-color: #4caf50;
		color: white;
		padding: 10px 15px;
		font-size: 16px;
		border: none;
		cursor: pointer;
		border-radius: 5px;
	}

	.dropdown-link {
		padding: 12px 16px;
		text-decoration: none;
		display: block;
		color: black;
		background-color: #f1f1f1;
		border-bottom: 1px solid #ddd; /* Add a border except for the last item */
	}

	.dropdown-link:hover {
		background-color: #ddd; /* Change color on hover */
	}

	.dropdown-link:last-child {
		border-bottom: none; /* No border for the last item */
	}
</style>
