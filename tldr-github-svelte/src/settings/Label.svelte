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
        color: white;
        display: inline-block;
        height: 20px;
        padding: .15em 4px;
        font-size: 12px;
        font-weight: 600;
        line-height: 15px;
        border-radius: 2px;
    }

    .blue {
        background: #3490DC;
    }

    .purle {
        background: #6574CD;
    }

    .green {
        background: #38C172;
    }

    .yellow {
        background: #FFED4A;
    }

    .red {
        background: #E3342F;
    }

    .pink {
        background: #F66D9B;
    }
</style>