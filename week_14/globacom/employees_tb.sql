--
-- PostgreSQL database dump
--

\restrict D18meWPbmfJxaPh060NMhDdaYtv1hoFNgim2kUAK3z6jFT2cp1rcir8PzoF6SrD

-- Dumped from database version 18.1
-- Dumped by pg_dump version 18.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: employees; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.employees (
    id integer NOT NULL,
    name text NOT NULL,
    no integer NOT NULL,
    sal real,
    age integer NOT NULL,
    phone real
);


ALTER TABLE public.employees OWNER TO postgres;

--
-- Data for Name: employees; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.employees (id, name, no, sal, age, phone) FROM stdin;
101	ALADE JOY	2	250000	33	8.0230897e+09
100	Mustapha Ali	3	175000	32	8.063286e+09
107	Alokwe Martin	7	380000	48	7.090083e+09
97	Dankade Aminat	5	550000	40	9.023689e+09
108	Josiah Joshua	1	120000	30	8.053189e+09
102	Mankinde Mary	2	450000	55	9.023488e+09
120	Adeleke Jane	4	200000	38	7.061046e+09
122	Osahon Mark	6	320000	44	8.02229e+09
117	Suleman Ajayi	3	800000	50	7.0300897e+09
104	Kuti Lawal	1	750000	35	9.14569e+09
\.


--
-- Name: employees employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.employees
    ADD CONSTRAINT employees_pkey PRIMARY KEY (id);


--
-- PostgreSQL database dump complete
--

\unrestrict D18meWPbmfJxaPh060NMhDdaYtv1hoFNgim2kUAK3z6jFT2cp1rcir8PzoF6SrD

