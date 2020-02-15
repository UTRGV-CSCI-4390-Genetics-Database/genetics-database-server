"use strict";

const inputElement = document.getElementById('input');
const responseElement = document.getElementById('response');

document.getElementById('executeQueryButton').onclick = executeQuery;

async function executeQuery() {
  const input = inputElement.value;
  const query = `SELECT * FROM ${input}`;
  const response = await fetch(`api/rawQuery/${encodeURIComponent(query)}`);
  responseElement.innerText = await response.text();
}