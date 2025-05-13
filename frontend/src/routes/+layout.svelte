<script lang="ts">
	import '../app.css';
	import Header from '$lib/components/Header.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import Footer from '$lib/components/Footer.svelte';
  import Profilepage from '$lib/components/Profilepage.svelte';

	let { children } = $props();
</script>

<div class="min-h-screen flex flex-col">
	<Profilepage />
	{@render children()}

	<Footer />
</div>
