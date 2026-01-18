<script>

</script>
<template>
    <div id="float-button-container" class="container">
        <button @click.stop="$emit('click')">
            <slot>+</slot>
        </button>
    </div>
</template>
<style scoped> 
#float-button-container {
    position: fixed;
    bottom: 24px;
    right: 24px;
    z-index: 1000;
}
</style>