<script>
	let { children } = $props();
</script>

<div>
  <div class="bladie bla">
  <Header />
  <Header />
  </div>
    <main>
      {@render children()}
    </main>
  <Footer />
</div>
