<script lang="ts">
	import { get_transactions } from '$lib/transactions';
	import { onMount } from 'svelte';

	let txns: any[];

	onMount(async () => {
		txns = await get_transactions();
	});
</script>

<h1>
	{#each txns as txn}
		<div>
			<span>{txn.payee_name}</span>
			<span>${txn.amount / 1000.0}</span>
		</div>
	{/each}
</h1>

<!-- TODO: Have the server run a simulation and return the results. Then display the results. -->
<!-- TODO: Either that or just get the latest simulation. -->
