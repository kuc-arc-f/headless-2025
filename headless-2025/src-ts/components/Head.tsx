import React, { useState, useEffect } from 'react';
//import { Routes, Route, Link } from 'react-router-dom';
import {Link } from 'react-router-dom';

function Page() {
    return (
    <div>
        <Link to="/foo" class="ms-2">Home</Link>
        <hr />
    </div>
    );
}
export default Page;
