<script>
    import Header from "$lib/components/Header.svelte";

    let cafes = [
        {name: "Hideout Café", address: "Heidelberglaan 15", image: "/hideout.png" },
        {name: "Cambridgebar", address: "Cambridgelaan 901", image: "/cambridge.png" },
        {name: "Café Het7de", address: "Heidelberglaan 7", image: "/7de.png" },
        {name: "Het Proeflokaal", address: "Padualaan 99", image: "/proef.png" }
    ];

    let selected = cafes[1];
</script>
<Header />
    <div class="flex-1 bg-gradient-plant content-center">
        <div class="max-w-5/10 mx-auto">
            <div class="h-auto bg-color-register mx-auto m-10 p-8 rounded-xl shadow-2xl shadow-black">
                <h1 class="font-bold text-2xl text-center">Selecteer een Nieuw Café:</h1>
                <p class="text-center">Kies één van de volgende locaties om</p>
                <p class="text-center">een aanmeldverzoek te sturen.</p>
                
                {#each cafes as cafe}
                <div
                  on:click={() => selected = cafe}
                  class="cursor-pointer flex items-center gap-3 p-4 my-3 rounded-lg shadow transition
                  {selected === cafe ? 'bg-yellow-200 border border-yellow-300 cafehover' : 'bg-white cafehover'}"
                >
                  <img
                    src={cafe.image}
                    alt={cafe.name}
                    class="w-10 h-10 bg-gray-200 rounded-full object-cover flex-shrink-0"
                  />
                  <div>
                    <p class="font-semibold">{cafe.name}</p>
                    <p class="text-sm text-gray-600">{cafe.address}</p>
                  </div>
                </div>
              {/each}
            </div>
        </div>
    </div>

<!--Here's supposed to be the UL with selecting a café-->


