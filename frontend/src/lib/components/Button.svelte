<script lang="ts">
    let { color, padding }: {color?: string, padding?: string} = $props();

    let color_style = "btn-green";
    let padding_style = "p-3 px-7.5"
    
    if (color === "orange") {
        color_style = "btn-orange"
    }

    if (color === 'red'){
        color_style = 'btn-red'
    }

    if (padding == "lg") {
        padding_style = "px-10 py-5"
    }
</script>
<button class="{color_style} w-auto max-h-6 {padding_style} border shadow-xl shadow-gray-300 rounded-lg flex items-center justify-center text-center hoverbutton">
    <slot/>
</button>