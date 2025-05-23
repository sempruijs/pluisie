<script lang="ts">
    import type { User } from "$lib/ts/domain";
    import { logout } from "$lib/ts/logout";
    import { Option } from "effect";

    export let user: User;

    const birthDate = "01-02-2003";
    const vereniging = "Indicium";
    const cafesStatus = "In behandeling...";
    const profileImageUrl = "";
</script>

<div class="flex justify-center w-full px-4 sm:px-6 lg:px-0">
    <div class="bg-white rounded-xl shadow-lg p-8 w-full max-w-2xl">
        <!-- Welcome message outside the border -->
        <h1 class="text-center text-2xl mb-6">Welkom Op Je Profiel!</h1>

        <!-- The rest of the profile content inside a single border -->
        <div class="border border-gray-500 p-6 rounded-lg">
            <!-- Side-by-side image and name inside the border -->
            <div class="flex items-center space-x-4 mb-6 justify-center">
                {#if profileImageUrl}
                    <img
                        src={profileImageUrl}
                        alt={user.name}
                        class="w-16 h-16 rounded-full object-cover"
                    />
                {:else}
                    <div
                        class="w-16 h-16 rounded-full bg-gray-300 flex items-center justify-center text-lg text-gray-600"
                    >
                        IMG
                    </div>
                {/if}

                <div class="text-xl">{user.name}</div>
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
                        >{user.iva}</a
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
        <button onclick={logout}>logut</button>
    </div>
</div>
