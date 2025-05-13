<script>
    export let name = "Bert Jan";
    export let birthDate = "01-02-2003";
    export let pronouns = "Hij/hem";
    export let iva = "Bert Jan’s IVA";
    export let vereniging = "Indicium";
    export let cafesStatus = "In behandeling...";
    export let profileImageUrl = "/icon.png"; // pass in the URL of the profile image
  </script>
  
  <div class="flex justify-center w-full">
    <div class="bg-white rounded-xl shadow-md p-6 w-full max-w-md border border-gray-300 font-serif">
      <div class="text-center text-xl font-bold mb-4">Welkom Op Je Profiel!</div>
  
      <div class="flex items-center space-x-3 mb-4">
        {#if profileImageUrl}
          <img
            src={profileImageUrl}
            alt={name}
            class="w-10 h-10 rounded-full object-cover"
          />
        {:else}
          <div class="w-10 h-10 rounded-full bg-gray-300 flex items-center justify-center text-xs text-gray-600">
            IMG
          </div>
        {/if}
  
        <div class="text-lg font-semibold">{name}</div>
      </div>
  
      <div class="space-y-2 text-sm">
        <div class="flex justify-between border-b pb-1">
          <span class="font-semibold">Geboortedatum:</span>
          <span class="font-bold">{birthDate}</span>
        </div>
        <div class="flex justify-between border-b pb-1">
          <span class="font-semibold">Mijn voornaamwoorden:</span>
          <span>{pronouns}</span>
        </div>
        <div class="flex justify-between border-b pb-1">
          <span class="font-semibold">IVA:</span>
          <span class="text-green-600 font-semibold italic">{iva}</span>
        </div>
        <div class="flex justify-between border-b pb-1">
          <span class="font-semibold">Vereniging:</span>
          <span class="font-medium">{vereniging}</span>
        </div>
        <div class="flex justify-between">
          <span class="font-semibold">Cafés:</span>
          <span class="text-orange-500 italic underline">{cafesStatus}</span>
        </div>
      </div>
    </div>
  </div>
  