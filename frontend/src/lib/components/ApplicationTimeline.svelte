<script>
  export let currentStep = 2;

  const steps = [
    "Verzoek Aangemaakt",
    "Verzoek Verstuurd",
    "In Behandeling",
    "Eindresultaat"
  ];
</script>

<div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-0">
  <div class="rounded-xl shadow-2xl p-6 sm:p-10 bg-white">
    <p class="text-2xl font-semibold mb-8 mt-5 text-center">Aanmeldverzoek In Behandeling...</p>

    <div class="border border-gray-300 rounded-lg p-4 sm:p-6">
      <!-- Status Label -->
      <div class="mb-6 inline-block bg-orange-300 px-3 py-1 text-base font-medium">
        Status:
      </div>

      <!-- Progress bar and steps -->
      <div class="relative">
        <!-- Background line -->
        <div class="absolute top-3 left-0 w-full h-2 bg-gray-300 z-0 rounded"></div>

        <!-- Progress line -->
        <div
          class="absolute top-3 left-0 h-2 bg-amber-300 z-10 rounded transition-all duration-300"
          style="width: {(currentStep - 0) / (steps.length - 0.10) * 100}%"
        ></div>

        <!-- Steps -->
        <div class="flex justify-between relative z-20">
          {#each steps as step, index}
            <div class="flex flex-col items-center w-1/4">
              <div class={`w-8 h-8 rounded-full border-4 ${index < currentStep ? 'bg-amber-200 border-amber-300' : 'bg-white border-gray-300'}`}></div>
              <span class="mt-3 text-sm text-center">{step}</span>
            </div>
          {/each}
        </div>
      </div>
    </div>

    <p class="text-xs mt-10 -mb-5 text-center">Hulp bij problemen? Contact op hoofdbar@cafehideout.nL</p>
  </div>
</div>
