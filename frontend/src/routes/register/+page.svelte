<script lang="ts">
    import Button from "$lib/components/Button.svelte";
    import Header from "$lib/components/Header.svelte";
    import { slide } from 'svelte/transition';
    import { Register } from "$lib/ts/register";
    import { provideServerConfig } from "$lib/ts/server";
    import { serverConfig } from "$lib/config/config.template";
    import { writable } from "svelte/store";
    import type { CreateUserRequest } from "$lib/ts/register";
    import { Effect } from "effect";
    import { base } from "$app/paths";
    import {get} from "svelte/store";

  let createUserRequest = writable({
    name: "",
    date_of_birth: "",
    is_super: false,
    iva: "",
    password: "",
    phone_number: "",
    email: "",
  });

    // $: createUserRequest.update(req => ({
    //     ...req,
    //     name: [firstName, infix, lastName].filter(Boolean).join(' ').trim()
    // }));
    
    const handleSubmit = () => {
      createUserRequest.update((req) => ({
        ...req,
        name: fullName,
        date_of_birth,
        email,
        phone_number,
        password,
      }));



    Effect.runPromise(
        provideServerConfig(serverConfig)(Register(get(createUserRequest))),
    )
        .then(() => {
            console.log("Created User");
            window.location.href = "/"
        })
            .catch((error) => {
                alert("Alert, creating user not succeeded." + error.message);
            });
        };

    $effect(() => {
        console.log($createUserRequest);
    })

    let dropdownContainer = "";

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

        date_of_birth = formatted;
    }

    let selected = 'Indicium';
    let options = ['Avanti', 'Indicium', 'Codex', 'SOG']
    let open = false;

    function selectOption(option) {
        selected = option;
        console.log("Selected option:", selected);
        open = false;
    }
        
    let showPopup = false;
    function handlePopup(evt){
        console.log ("test mf's")
        showPopup = true;

        setTimeout(()=> {
            showPopup = false;
             window.location.href = "/"
        }, 9000);
    }

    function handleFileUpload(event) {
        const file = event.target.files[0];
        if (file) {
            console.log("Bestand geüpload:", file.name);
        }
    }
</script>
<Header />
<div class="flex-1 bg-gradient-plant pt-16">
    <div class="center-v center-h mt-10">
        <img src="/hideoutlogo.png" alt="Logo" />
    </div>
    <div class="max-w-8/10 mx-auto">
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
            <h1 class="font-bold text-3xl">Aanmeld Formulier</h1>
        </div>
        <div class="py-8 sha">
            <div class="flex gap-2 w-full">
                <div class="form-control w-2/6">
                    <h3 class="form-label textcontrast">Voornaam:</h3>
                    <input type="text" required
                        placeholder="Voornaam"
                        class="w-full h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 rounded-lg outline-none selecttext mb-3" 
                        bind:value={$createUserRequest.name}
                    />
                </div>
            
                <div class="form-control w-1/6">
                    <h3 class="form-label textcontrast">T.V.</h3>
                    <input type="text"
                        placeholder="T.V."
                        class="w-full h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 rounded-lg outline-none selecttext mb-3" 
                    />
                </div>
            
                <div class="form-control w-3/6">
                    <h3 class="form-label textcontrast">Achternaam:</h3>
                    <input type="text" required
                        placeholder="Achternaam"
                        class="w-full h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 rounded-lg outline-none selecttext mb-3" 
                    />
                </div>
            </div>
            
            <div class="form-control">
                <h3 class="form-label textcontrast">Geboortedatum:</h3>
            <input
                type="date" required
                class="w-30 h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 rounded-lg outline-none selecttext mb-3" 
                placeholder="dd-mm-jjjj"
                maxlength="10"
                bind:value={$createUserRequest.date_of_birth}
                />
        </div>
            <div class="form-control">
                <h3 class="form-label textcontrast">Persoonlijke e-mail:</h3>
        <input
            type="text" required
            placeholder="Persoonlijke e-mail"
            class="h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 rounded-lg outline-none selecttext mb-3" 
            bind:value={$createUserRequest.email}
        />
        </div>
        <div class="form-control">
            <h3 class="form-label textcontrast">Wachtwoord:</h3>
    <input type="text" required
            placeholder="Wachtwoord"
            class="h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 rounded-lg outline-none selecttext mb-3" 
            bind:value={$createUserRequest.password}
            />
    </div>

           
                    <h3 class="textcontrast">Telefoonnummer:</h3>
            <input type="text"
                    placeholder="Telefoonnummer"
                    class="w-65 h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 rounded-lg outline-none selecttext mb-3" 
                    bind:value={$createUserRequest.phone_number}
                    />
            
            <div class="w-full mb-3">
                <label class="block mb-1 text-black"><h3 class="textcontrast">Selecteer je vereniging:</h3></label>
            
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
                            class="px-4 py-2 hover:bg-gray-200 hover:font-bold selectlist cursor-pointer"
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
            <div class="form-control">
                <h3 class="form-label textcontrast">IVA uploaden:</h3>
                    <div class="w-full mx-auto h-50 bg-dragfile shadow-2xl border-2 border-dashed border-gray-400 mb-3">
                        <div class="text-center my-10 justify-items-center text-gray-500">
                            <p>Sleep hier je bestanden naar toe</p>
                            <p class="my-3">Of:</p>
                            <label class="curser-pointer inline-block bg-yellow-500 text-white py-2 px-4 rounded-lg shadow hover:bg-yellow-600">
                                Upload vanaf je computer
                                <input
                                    type="file"
                                    accept=".pdf,.jpg,.jpeg,.png"
                                    class="hidden"
                                    on:change={(e) => {
                                        const file = e.target.files[0];
                                        if (file) {
                                           const reader = new FileReader();
                                           reader.onload = () =>{
                                            const base64String = reader.result as string;
                                            createUserRequest.update(req => ({
                                                ...req,
                                                iva: base64String,
                                            }));
                                            console.log("bestand geüpload als string:",base64String.substring(0, 100));
                                           }
                                           reader.readAsDataURL(file);
                                        }
                                    }}
                                />
                            </label>
                        </div>
                    </div>
                </div>
            <div class="my-5">
            <Button color="orange" padding="lg" width="max"
            on:click={() => {
                handlePopup();
                handleSubmit();
            }}
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
