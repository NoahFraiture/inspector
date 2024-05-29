<script lang="ts">
	import { graphNameStore, graphName } from '../store';
	import { onMount } from 'svelte';

	interface UserStats {
		visits: number;
		purchases: number;
		comments: number;
	}

	interface UserData {
		[key: string]: UserStats;
	}

	let userData: UserStats = {
		visits: 0,
		purchases: 0,
		comments: 0
	};

	const fetchUserData = async (userName: string) => {
		// Simulating fetching user data
		const data: UserData = {
			'Option 1': { visits: 120, purchases: 30, comments: 50 },
			'Option 2': { visits: 80, purchases: 20, comments: 40 },
			'Option 3': { visits: 150, purchases: 45, comments: 70 }
		};
		return (
			data[userName] || {
				visits: 0,
				purchases: 0,
				comments: 0
			}
		);
	};

	onMount(async () => {
		graphNameStore.subscribe(async ($graphName) => {
			userData = await fetchUserData($graphName);
		});
	});
</script>

<!-- Display the user statistics -->
<div class="stats-container">
	<h3>Statistics for {$graphName}</h3>
	<p>Visits: {userData.visits}</p>
	<p>Purchases: {userData.purchases}</p>
	<p>Comments: {userData.comments}</p>
</div>

<style>
	.stats-container {
		margin-top: 20px;
		padding: 20px;
		background-color: #f9f9f9;
		border-radius: 8px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}
</style>
