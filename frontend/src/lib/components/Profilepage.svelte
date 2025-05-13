<script>
    export let name = "Bert Jan";
    export let birthDate = "01-02-2003";
    export let pronouns = "Hij/hem";
    export let iva = "Bert Jan’s IVA";
    export let vereniging = "Indicium";
    export let cafesStatus = "In behandeling...";
    export let profileImageUrl = "";
  </script>
  
  <div class="flex justify-center w-full">
    <div class="bg-white rounded-xl shadow-md p-6 w-full max-w-md border border-gray-300">
      <div class="text-center text-xl font-semibold mb-4">Welkom Op Je Profiel!</div>
  
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
  
        <div class="text-lg">{name}</div>
      </div>
  
      <div class="space-y-2 text-sm">
        <div class="flex justify-between pb-1">
          <span>Geboortedatum:</span>
          <span>{birthDate}</span>
        </div>
        <div class="flex justify-between pb-1">
          <span>Mijn voornaamwoorden:</span>
          <span>{pronouns}</span>
        </div>
        <div class="flex justify-between pb-1">
          <span>IVA:</span>
          <span class="text-green-600 italic">{iva}</span>
        </div>
        <div class="flex justify-between pb-1">
          <span>Vereniging:</span>
          <span>{vereniging}</span>
        </div>
        <div class="flex justify-between">
          <span>Cafés:</span>
          <span class="text-orange-500 italic underline">{cafesStatus}</span>
        </div>
      </div>
    </div>
  </div>
  