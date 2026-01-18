<script>

</script>
<template>
    <div id="menu" class="container float-menu">
        <h2>Menu Component</h2>
        <button @click="$emit('close')">Close Menu</button>
        <button @click="$emit('new-work')">New Work</button>
        <button @click="$emit('option-2')">Read</button>
        <button @click="$emit('option-3')">Option 3</button>
    </div>
</template>
<style scoped>
.float-menu {
    position: fixed;
    bottom: 80px;
    right: 24px;
    width: 200px;
    background-color: white;
    border: 1px solid #ccc;
    border-radius: 4px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
    padding: 16px;
    z-index: 1000;
}
</style>