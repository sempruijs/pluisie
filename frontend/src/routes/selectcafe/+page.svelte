<script>
	import { slide } from 'svelte/transition';
    import Header from "$lib/components/Header.svelte";
    import Button from "$lib/components/Button.svelte";
    import { tick } from "svelte";

    let cafes = [
        {name: "Hideout Café", address: "Heidelberglaan 15", image: "/hideout.png" },
        {name: "Cambridgebar", address: "Cambridgelaan 901", image: "/cambridge.png" },
        {name: "Café Het7de", address: "Heidelberglaan 7", image: "/7de.png" },
        {name: "Het Proeflokaal", address: "Padualaan 99", image: "/proef.png" }
    ];

    let selected = cafes[1];
    let open = false;
    let dropdownContainer;

    function opendropDown() {
        open = !open;
        if (open) {
          setTimeout(() => {
            dropdownContainer?.scrollIntoView({
              behavior: 'smooth',
              block: 'start'
            });
          }, 50);
        }
    }

    let showPopup = false;
        function handleSubmit(){
            console.log ("test mf's")
            showPopup = true;

            setTimeout(()=> {
                showPopup = false;
                window.location.href = "/selectcafe/calendar"
            }, 3250);
        }

</script>
<Header />
    <div class="flex-1 bg-gradient-plant flex items-center">
        <div class="max-w-2xl w-full mx-auto p-5">
            <div class="h-auto bg-color-register mx-auto m-10 p-8 rounded-xl shadow-2xl shadow-black">
                <h1 class="font-bold text-2xl text-center">Selecteer een Nieuw Café:</h1>
                <p class="text-center">Kies één van de volgende locaties om</p>
                <p class="text-center">een aanmeldverzoek te sturen.</p>
                
                {#each cafes as cafe} 
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <div
                  on:click={async () => {
                   if (selected === cafe) {
                    open = !open; 
                   } else {
                    selected = cafe;
                    open = true;
                   }

                   if (open) {
                    await tick();
                      dropdownContainer?.scrollIntoView({
                        behavior: 'smooth',
                        block: 'start'
                      });
                    }
                   }
                  }
                  class="cursor-pointer flex items-center gap-3 p-4 my-3 rounded-lg shadow transition cafehover
                  {selected === cafe ? 'cafehover-selected' : 'bg-white'}"
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
              {#if open}
              <div bind:this={dropdownContainer}
                in:slide
                out:slide
                class="mt-6 p-6 bg-white rounded-xl shadow">
                <h2 class="text-lg font-semibold mb-2">Je hebt gekozen voor:</h2>
                <p class="mb-4">
                  <strong>{selected.name}</strong>, {selected.address}
                </p>

                <label for="motivation" class="block mb-2 text-gray-700">Licht je motivatie toe:</label>
                <textarea
                  id="motivation"
                  class="w-full p-3 border border-gray-300 rounded-lg focus: outline-none focus:ring-2 focus:ring-yellow-400"
                  rows="4"
                  placeholder="Waarom wil je je aanmelden bij dit café?"
                 ></textarea> 

                 <Button
                 color="orange"
                 class="mt-3"
                  on:click={handleSubmit}
                  >
                  
                    Verzenden
              </Button>
              {#if showPopup}
              <div class="popup-container">
                  <div class="popup">
                    <p>Bedankt voor je aanmelding!</p>
                    <p class="pt-3 font-semibold">
                      Met vriendelijke groet,
                  </p>
                  <p>
                      Het bestuur van Science Café Hideout
                  </p>
                  </div>
              </div>
            {/if}
              </div>
            {/if}
            </div>
        </div>
    </div>

<!--Here's supposed to be the UL with selecting a café-->


