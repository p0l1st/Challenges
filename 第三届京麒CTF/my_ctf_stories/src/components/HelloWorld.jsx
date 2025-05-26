import React, { useEffect, useState } from 'react';

function HelloWorld({ text }) {
  const [flag, setFlag] = useState('');

  useEffect(() => {
    const allCookies = document.cookie;  
    const pairs = allCookies.split('; ');  
    const flagPair = pairs.find(part => part.startsWith('flag='));  
    const flagValue = flagPair
        ? decodeURIComponent(flagPair.split('=')[1])
        : '';  
    if (flagValue) {
        setFlag(flagValue);
    } else {
        setFlag('flag{hello_world}');
    }
  }, []);

  return (
    <div className="hello-container">
      <h1>{text}</h1>
      {flag && (
        <div
          id="secret"
          data-flag={flag}
        />
      )}
    </div>
  );
}

export default HelloWorld;
