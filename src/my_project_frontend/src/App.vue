<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';

const rates = ref([])
const iloscWaluty = ref([])

const getDataNBP = async () => {
  const response = await fetch("https://api.nbp.pl/api/exchangerates/tables/A/?format=json");
  const jsonData = await response.json();
  rates.value = jsonData[0].rates;
  iloscWaluty.value = jsonData[0].rates.map(() => 0)
}


const calculateCost = async (iloscWaluty, index) => {
  const ilosc = BigInt(iloscWaluty)
  const cena = BigInt(rates.value[index].mid * 10e16)

  const koszt = await my_project_backend.calculate_currency_price(ilosc, cena)
  return koszt / BigInt(10e16)
}

const kupWalute = async () => {
  const koszta = await Promise.all(iloscWaluty.value.map((ilosc, index) => calculateCost(ilosc, index)))
  
  const innit = BigInt(0);
  const sum = await koszta.reduce(
    async (ac, val) => { 
      ac = await ac;
      return ac + val
    },
    innit,
  );
  document.getElementsByClassName("res")[0].innerHTML = `<br>Wszyskie waluty kosztują: ` + sum.toString();
  console.log(sum)
}

const onChange = (e, index) => {
  iloscWaluty.value[index] = e.target.value
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
        <th>ilość waluty do kupienia</th>
        <th>Ilosc za 1 dolara</th>
      </tr>
      <tr v-for="(rate, index) in rates">
        <td>{{ rate.currency }}</td>
        <td>{{ rate.code }}</td>
        <td>{{ rate.mid }}</td>
        <td><input type="number" @change="(e) => onChange(e, index)"/></td>
        <td></td>
      </tr>
    </table>
    <button @click="kupWalute(index)">Kup</button>
    <a class="res"></a>
  </main>
</template>
