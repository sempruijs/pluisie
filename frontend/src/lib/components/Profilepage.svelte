<script lang="ts">
    import type { User } from "$lib/ts/domain";
    import { Effect, Option } from "effect";
    import { GetUser } from "$lib/ts/GetUser";
    import { provideServerConfig } from "$lib/ts/server";
    import { serverConfig } from "$lib/config/config.template";
    import { onMount } from "svelte";

    const user: User = {
        name: "bla",
        email: "bla@bla.nl",
        iva: "",
        is_super: false,
    };

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

    const birthDate = "01-02-2003";
    const vereniging = "Indicium";
    const cafesStatus = "In behandeling...";
    const profileImageUrl = "";
</script>

{#if Option.isSome(state.user)}
    <div class="flex justify-center w-full px-4 sm:px-6 lg:px-0">
        <div class="bg-white rounded-xl shadow-lg p-8 w-full max-w-2xl">
            <!-- Welcome message outside the border -->
            <div class="text-center text-2xl mb-6">Welkom Op Je Profiel!</div>

            <!-- The rest of the profile content inside a single border -->
            <div class="border border-gray-500 p-6 rounded-lg">
                <!-- Side-by-side image and name inside the border -->
                <div class="flex items-center space-x-4 mb-6 justify-center">
                    {#if profileImageUrl}
                        <img
                            src={profileImageUrl}
                            alt={state.user.value.name}
                            class="w-16 h-16 rounded-full object-cover"
                        />
                    {:else}
                        <div
                            class="w-16 h-16 rounded-full bg-gray-300 flex items-center justify-center text-lg text-gray-600"
                        >
                            IMG
                        </div>
                    {/if}

                    <div class="text-xl">{state.user.value.name}</div>
                </div>

                <!-- Info section inside the border -->
                <div class="space-y-3 text-base">
                    <div class="flex justify-between pb-2">
                        <span>Geboortedatum:</span>
                        <span>{birthDate}</span>
                    </div>
                    <div class="flex justify-between pb-2">
                        <span>IVA:</span>
                        <!-- Make "IVA" clickable -->
                        <a
                            href="/iva-details"
                            class="text-green-600 italic hover:underline"
                            >{state.user.value.iva}</a
                        >
                    </div>
                    <div class="flex justify-between pb-2">
                        <span>Vereniging:</span>
                        <span>{vereniging}</span>
                    </div>
                    <div class="flex justify-between">
                        <span>Cafés:</span>
                        <!-- Make "Cafés" clickable -->
                        <a
                            href="/profile/cafes-status"
                            class="text-orange-500 italic underline hover:underline"
                            >{cafesStatus}</a
                        >
                    </div>
                </div>
            </div>
        </div>
    </div>
{:else}
    <h1>loading user data</h1>
{/if}
