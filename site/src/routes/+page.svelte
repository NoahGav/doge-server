<script lang="ts">
	import { get_transactions } from '$lib/transactions';
	import { onMount } from 'svelte';

	let data: any[];

	onMount(async () => {
		data = await get_transactions();
	});
</script>

<h1>{JSON.stringify(data)}</h1>
