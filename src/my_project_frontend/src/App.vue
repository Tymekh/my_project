<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';

const getDataNBP = async () => {
  const response = await fetch("https://api.nbp.pl/api/exchangerates/tables/A/?format=json");
  const jsonData = await response.json();
  console.log(jsonData);
  const rates = jsonData[0].rates;
  console.log(rates)
}
getDataNBP();
</script>

<template>
  <main>

  </main>
</template>
