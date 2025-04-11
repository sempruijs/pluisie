<script>
  let showLogin = true;

  funtion goToMain() {
    showLogin = false;
  }
    
</script>

{#if showLogin}

  <div>
    <h1>Login Page</h1>
    <button on:click={goToMain}>go to main page</button>
  </div>
  {:else}
    <div>
      <button on:click={() => showLogin = true}>back to Login</button>
    </div>
  {/if}
