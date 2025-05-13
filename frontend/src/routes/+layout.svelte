<script lang="ts">
	import '../app.css';
	import Header from '$lib/components/Header.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import Footer from '$lib/components/Footer.svelte';
    import ApplicationTimeline from '$lib/components/ApplicationTimeline.svelte';

	let { children } = $props();
</script>

<div class="min-h-screen flex flex-col">
	{@render children()}
	<Footer />
</div>
