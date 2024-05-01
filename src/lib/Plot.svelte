<script lang="ts">
	import { onMount } from 'svelte';
	import Chart from 'chart.js/auto';
	import { graphName } from '../store';

	let ctx: HTMLCanvasElement;
	let chart: Chart;
	$: $graphName, updateGraph();

	function updateGraph() {
		if (chart) {
			chart.destroy();
		}
		chart = new Chart(ctx, {
			type: 'line',
			data: {
				labels: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], // X-axis labels
				datasets: [
					{
						label: $graphName,
						data: [100, 200, 300, 250, 500, 450, 600, 550, 700, 650], // Your data
						borderColor: 'rgb(255, 99, 132)',
						backgroundColor: 'rgba(255, 99, 132, 0.5)'
					},
					{
						label: 'Series 2',
						data: [400, 300, 200, 300, 400, 500, 400, 300, 500, 400],
						borderColor: 'rgb(54, 162, 235)',
						backgroundColor: 'rgba(54, 162, 235, 0.5)'
					}
				]
			},
			options: {
				scales: {
					y: {
						beginAtZero: true
					}
				}
			}
		});
	}

	onMount(() => updateGraph());
</script>

<canvas bind:this={ctx}></canvas>

<style>
	/* Assuming you have global styles for nav, main from the previous example */
	canvas {
		width: 100%;
		height: 100%;
		padding: 20px; /* Space around the canvas */
	}
</style>
