<script lang="ts">
	import Dropdown from '$lib/Dropdown.svelte';
	import Plot from '$lib/Plot.svelte';
	import StatsSummary from '$lib/StatsSummary.svelte';
	import { graphName } from '../../store';

	let pageTitle = 'You are : ';
</script>

<h1>{pageTitle + $graphName}</h1>

<Dropdown />
<Plot />
<StatsSummary />
