<script lang="ts">
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher()

    let { color, padding, width, class: className }: {color?: string, padding?: string, width?: string, class?: string} = $props();

    let color_style = "btn-green";
    let padding_style = "p-3 px-7.5";
    let width_style = "w-auto";
    
    if (color === "orange") {
        color_style = "btn-orange"
    }

    if (color === 'red'){
        color_style = 'btn-red'
    }

    if (color === 'yellow'){
        color_style = 'btn-yellow'
    }

    if (padding == "lg") {
        padding_style = "px-10 py-5"
    }

    if (width == "max") {
        width_style = 'w-full'
    }
</script>

<button onclick={(e) => dispatch("click", e)} class="{color_style} {width_style} max-h-6 {padding_style} border shadow-xl shadow-gray-300 rounded-lg flex items-center justify-center text-center hoverbutton {className || ''}">
    <slot/>
</button>