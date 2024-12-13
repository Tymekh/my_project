<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';

const rates = ref([])

const getDataNBP = async () => {
  const response = await fetch("https://api.nbp.pl/api/exchangerates/tables/A/?format=json");
  const jsonData = await response.json();
  console.log(jsonData);
  rates.value = jsonData[0].rates;
  console.log(rates)
}
getDataNBP();
</script>

<template>
  <main>
    <table>
      <tr>
        <th>Nazwa waluty</th>
        <th>Kod waluty</th>
        <th>Cena</th>
      </tr>
      <tr v-for="rate in rates">
        <td>{{  rate.currency  }}</td>
        <td>{{  rate.code  }}</td>
        <td>{{  rate.mid  }}</td>
      </tr>
    </table>
  </main>
</template>
