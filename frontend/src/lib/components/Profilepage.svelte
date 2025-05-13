<script>
    export let name = "Bert Jan";
    export let birthDate = "01-02-2003";
    export let pronouns = "Hij/hem";
    export let iva = "Bert Jan’s IVA";
    export let vereniging = "Indicium";
    export let cafesStatus = "In behandeling...";
  </script>
  
  <div class="bg-white rounded-xl shadow-md p-6 w-full max-w-md border border-gray-300 font-serif">
    <div class="text-center text-xl font-bold mb-4">Welkom Op Je Profiel!</div>
  
    <div class="flex items-center space-x-3 mb-4">
      <div class="bg-gray-200 rounded-full w-10 h-10 flex items-center justify-center">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5.121 17.804A13.937 13.937 0 0112 15c2.21 0 4.283.535 6.121 1.484M15 10a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
      </div>
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
  