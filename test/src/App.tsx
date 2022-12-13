import { StrictMode, useEffect, useState } from 'react';
import './App.css';
import { program_node } from './output/ast';
import { Program } from './output/components';

function App() {


  function update(expr: program_node){
    console.log(expr);
  }

  return (
    <StrictMode>
    <div className="App">
      <Program set={update}/>
    </div>
    </StrictMode>
  );
}

export default App;
