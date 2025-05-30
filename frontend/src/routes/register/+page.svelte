<script>
    import Button from "$lib/components/Button.svelte";
    import Header from "$lib/components/Header.svelte";
    import { slide } from 'svelte/transition';
   
    let geboortedatum = '';
    let dropdownContainer;
    let voornaam = "";
    let tussenvoegsels = "";
    let achternaam = "";
    let email = "";
    let telefoonnummer = "";

    function opendropDown() {
        open = !open
        if (open)(
            dropdownContainer.scrollIntoView({
                behavior:'smooth'
     })
        )
    }
    function formatGeboortedatum(e) {
        let input = e.target.value.replace(/\D/g, '');
        if (input.length > 8) input = input.slice(0, 8);

        let day = input.slice(0, 2);
        let month = input.slice(2, 4);
        let year = input.slice (4, 8);
        
        let formatted = day;
        if (month) formatted += '-' + month;
        if (year) formatted += '-' + year;

        geboortedatum = formatted;
        }

        let selected = 'Indicium';
        let options = ['Avanti', 'Indicium', 'Codex', 'SOG', 'test', 'test', 'test', 'test']
        let open = false;

        function selectOption(option) {
            selected = option;
            open = false;
        }
        
        let showPopup = false;
        function handleSubmit(evt){
            console.log ("test mf's")
            showPopup = true;

            fetch("http://localhost:8000/users", {
                method: "POST",
                body: JSON.stringify({
                    naam: voornaam + " " + tussenvoegsels + " " + achternaam,
                    email,
                })
            })

            setTimeout(()=> {
                showPopup = false;
                 window.location.href = "/"
            }, 9000);
        }

</script>
<Header />
<div class="flex-1 bg-gradient-plant pt-16">
    <div class="center-v center-h mt-10">
        <img src="/hideoutlogo.png" alt="Logo" />
    </div>
    <div class="max-w-5/10 mx-auto">
        <div class="h-auto bg-color-register mx-auto m-10 p-8 rounded-xl shadow-2xl shadow-black">
            <p>
                Bedankt voor je interesse!
            </p>
            <p class="py-3">
                Wat leuk dat je vrijwilliger wilt worden bij Science Café Hideout!
                Dankzij enthousiaste studenten zoals jij kunnen we elke dag een gezellige plek maken voor studenten en docenten op de uithof.
            </p>
            <p class="py-3 font-semibold"> Belangrijk: ICA-certificaat </p>
            <p>
                Voordat je achter de bar mag staan, moet je het IVA-certificaat (Instructie Verantwoord Alcohol schenken) halen. Deze gratis e-learning duurt ongeveer 15 minuten en kan je doen via de website van NOC*NSE.
            </p>
            <p class="py-3">
                Hier leer je hoe je veilig en verantwoord alcohol schenkt. Zodra je de e-learning correct afrond, heb je je IVA!
            </p>
        </div>
        <div>
            <h1 class="font-bold text-3xl">Aanmeldformulier</h1>
        </div>
        <div class="py-8 sha">
            <div class="form-control">
                <h3 class="form-label">Voornaam:</h3>
            <input type="text" required
                    class="w-full h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 rounded-lg outline-none selecttext mb-3" 
                    bind:value={voornaam}
                    />
            </div>
                    <h3>Tussenvoegsels:</h3>
            <input type="text"
                    class="w-full h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 rounded-lg outline-none selecttext mb-3" 
                    bind:value={tussenvoegsels}
                    />
            <div class="form-control">
                    <h3 class="form-label">Achternaam:</h3>
            <input type="text" required
                    class="w-full h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 rounded-lg outline-none selecttext mb-3" 
                    bind:value={achternaam}
                    />
            </div>
            <div class="form-control">
                <h3 class="form-label">E-mail:</h3>
        <input type="text" required
                class="w-full h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 rounded-lg outline-none selecttext mb-3" 
                bind:value={email}
                />
        </div>
            <div class="form-control">
                    <h3 class="form-label">Geboortedatum:</h3>
            <input type="text" required
                    class="w-full h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 rounded-lg outline-none selecttext mb-3" 
                    placeholder="dd-mm-jjjj"
                    maxlength="10"
                    bind:value={geboortedatum}
                    on:input={formatGeboortedatum}
                    />
            </div>
                    <h3>Telefoonnummer:</h3>
            <input type="text"
                    class="w-full h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 rounded-lg outline-none selecttext mb-3" 
                    bind:value={telefoonnummer}
                    />
            <div class="form-control">
            <h3 class="form-label">IVA uploaden:</h3>
                <div class="w-full mx-auto h-50 bg-dragfile shadow-2xl border-2 border-dashed border-gray-400 mb-3">
                    <div class="text-center my-10 justify-items-center text-gray-500">
                        <p>Sleep hier je bestanden naar toe</p>
                        <p class="my-3">Of:</p>
                        <Button color="yellow">
                            Upload vanaf je computer
                        </Button>
                    </div>
                </div>
            </div>
            
            <div class="w-full">
                <label class="block mb-1 text-black"><h3>Selecteer je vereniging:</h3></label>
            
            <div class="relative">
                <button
                class="w-full min-h-10 bg-gray-200 border px-3 border-gray-400 rounded-lg outline-none text-left cursor-pointer"
                on:click={() =>
                    opendropDown()
                }
                >
                    {selected}
                    <span class="absolute right-3 transition-transform duration-500"
                            class:-rotate-180={open}
                    >
                    &#x21e7;
                    </span>
                </button>
            <div bind:this={dropdownContainer}>
                {#if open}
                    <ul class="w-full mt-1 bg-white border border-gray-400 rounded-lg shadow max-h-40 overflow-y-auto"
                    transition:slide
                    >
                        
                        {#each options as option}
                        <li
                            class="px-4 py-2 hover:bg-gray-200 selectlist cursor-pointer"
                            on:click={() => selectOption(option)}
                        >
                            {option}
                        </li>
                    {/each}
                    </ul>
              {/if}
            </div>
             </div>
            </div>
            <div class="my-5">
            <Button color="orange" padding="lg" width="max"
            on:click={handleSubmit}
            >
                Verstuur aanmelding
            </Button>
            </div>
            {#if showPopup}
                <div class="popup-container">
                    <div class="popup">
                    <p>
                        We zullen zo spoedig mogelijk contact met je opnemen om je één dagje in te werken en om je te verwelkomen in onze fantastische vrijwilligers groep.
                    </p>
                    <p class="mt-2">
                        Nogmaals, bedankt voor je bereidheid om bij te dragen aan het succes van het Science Café Hideout!
                    </p>
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
    </div>
</div> 
