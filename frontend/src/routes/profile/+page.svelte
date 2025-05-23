<script lang="ts">
  import { Effect } from "effect";
  import Header from "$lib/components/Header.svelte";
  import Profile from "$lib/components/Profilepage.svelte";
  import { GetUser } from "$lib/ts/GetUser";
  import { provideServerConfig } from "$lib/ts/server";
  import { serverConfig } from "$lib/config/config.template";
  import { onMount } from "svelte";
  import { Option } from "effect";
  import type { User } from "$lib/ts/domain";

  const state = $state({
    user: Option.none<User>(),
  });

  const fetchUser = (jwt: string) => {
    Effect.runPromise(provideServerConfig(serverConfig)(GetUser(jwt))).then(
      (user: User) => {
        state.user = Option.some(user);
      },
    );
  };

  onMount(() => {
    const jwt = localStorage.getItem("jwt");
    if (jwt == null) {
      window.location.href = "/";
    } else {
      fetchUser(jwt);
    }
  });
</script>

<main class="flex1 center-v center-h flex-1 bg-gradient">
  <Header />
  {#if Option.isSome(state.user)}
    <Profile user={state.user.value} />
  {:else}
    <h1>Loading</h1>
  {/if}
</main>
