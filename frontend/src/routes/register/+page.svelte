<script>
    let geboortedatum = '';

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
</script>

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
        <div class="">
            <h1 class="font-bold text-3xl">Aanmeld formulier</h1>
        </div>
        <div class="py-8 sha">
                <h3>Voornaam:</h3>
            <input type="text"
                    class="w-full h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 rounded-lg outline-none selecttext mb-3" 
                    />
                    <h3>Tussenvoegsels:</h3>
            <input type="text"
                    class="w-full h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 rounded-lg outline-none selecttext mb-3" 
                    />
                    <h3>Achternaam:</h3>
            <input type="text"
                    class="w-full h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 rounded-lg outline-none selecttext mb-3" 
                    />
                    <h3>Geboortedatum:</h3>
            <input type="text"
                    class="w-full h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 rounded-lg outline-none selecttext mb-3" 
                    placeholder="dd-mm-jjjj"
                    maxlength="10"
                    bind:value={geboortedatum}
                    on:input={formatGeboortedatum}
                    />
                    <h3>Telefoon nummer:</h3>
            <input type="text"
                    class="w-full h-7.5 bg-gray-200 shadow-xl border px-3 border-gray-400 rounded-lg outline-none selecttext mb-3" 
                    />
            <h3>IVA uploaden:</h3>
            <div class="w-full h-50 bg-dragfile shadow-2xl">

            </div>
        </div>   
    </div>
</div> 
