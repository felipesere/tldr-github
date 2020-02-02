<script>
    export let value;

    const colors = [
        "blue",
        "purple",
        "green",
        "yellow",
        "red",
        "pink",
    ];

    function hash(val) {
        let num = 0;
        for (let i = 0; i < val.length; i++) {
            num += val.charCodeAt(i)
        }
        return num
    }

    $: idx = hash(value) % colors.length;
    $: color = colors[idx]
</script>

<div class={color}>
    {value}
</div>

<style>
    div {
        @apply text-white inline-block p-1 rounded-sm text-xs;
    }

    .blue {
        @apply bg-blue-600;
    }

    .purple {
        @apply bg-purple-600;
    }

    .green {
        @apply bg-green-600;
    }

    .yellow {
        @apply bg-yellow-600;
    }

    .red {
        @apply bg-red-600;
    }

    .pink {
        @apply bg-pink-600;
    }
</style>