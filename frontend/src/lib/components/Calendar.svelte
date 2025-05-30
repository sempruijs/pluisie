<script>
	import Month from './Month.svelte';
</script>

<div class="w-full h-auto pb-4 max-w-lg bg-white rounded rounded-lg">
    <h1 class=" p-8 text-center font-bold text-2xl">Selecteer een datum & tijd</h1> 
    <Month />
    <div class="border-2 rounded-lg p-4 m-5 max-w-md">
        <div class="flex flex-col sm:flex-row sm:items-center sm:justify-start gap-4">
            
          <div class="flex items-center gap-2">
            <div class="w-8 h-8 rounded-full bg-gray-300"></div>
            <p class="text-sm sm:text-base">= Onbeschikbaar</p>
          </div>
      
          <div class="flex items-center gap-2">
            <div class="w-8 h-8 rounded-full bg-green-200"></div>
            <p class="text-sm sm:text-base">= Beschikbaar</p>
          </div>
      
          <div class="flex items-center gap-2">
            <div class="w-8 h-8 rounded-full bg-red-300"></div>
            <p class="text-sm sm:text-base">= Vol</p>
          </div>
        </div>
      </div>
      
</div>
