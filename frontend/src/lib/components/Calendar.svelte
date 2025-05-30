<script>
	import Month from './Month.svelte';
</script>

<div class="w-full h-auto pb-4 max-w-lg bg-white rounded rounded-lg">
    <h1 class=" p-8 text-center font-bold text-2xl">Selecteer een datum & tijd</h1> 
    <Month />
    <div class="border border-2 max-w-5/10 ml-5 m-5">
    <div class="flex justify-between items-center max-w-42 ml-5 mt-5">
        <div class="w-8 h-8 rounded-full bg-gray-300">
    
        </div>
        <p>
            = Onbeschikbaar
        </p>
    </div>
    <div class="flex justify-between items-center max-w-37 ml-5 mt-5">
        <div class="w-8 h-8 rounded-full bg-green-200">
    
        </div>
        <p>
            = Beschikbaar
        </p>
    </div>
    <div class="flex justify-between items-center max-w-20 ml-5 mt-5 mb-5">
        <div class="w-8 h-8 rounded-full bg-red-300">
    
        </div>
        <p>
            = Vol
        </p>
    </div>
    </div>
</div>
