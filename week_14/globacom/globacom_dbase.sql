--
-- PostgreSQL database dump
--

\restrict 9QLPnaOUZQGBrqAluRBHwL4rgcOALKXDSiz6wwobndVcvgJizOCxAMmDU1rzHbj

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
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    id integer NOT NULL,
    no integer NOT NULL,
    name text NOT NULL,
    location text NOT NULL,
    pno integer
);


ALTER TABLE public.department OWNER TO postgres;

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
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    no integer NOT NULL,
    name text NOT NULL,
    duration text NOT NULL,
    id integer
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (id, no, name, location, pno) FROM stdin;
108	1	Administration	Ikeja	44
101	2	Account	Egbeda	11
100	3	Packaging	Ajah	44
120	4	Research	V.I	33
97	5	Account	Magodo	22
122	6	Operations	Mile 2	44
107	7	Packaging	Ketu	55
\.


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
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (no, name, duration, id) FROM stdin;
11	A	9 Months	102
22	B	14 Months	97
33	C	16 Months	120
44	D	25 Months	108
55	E	9 Months	107
\.


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (id);


--
-- Name: employees employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.employees
    ADD CONSTRAINT employees_pkey PRIMARY KEY (id);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (no);


--
-- PostgreSQL database dump complete
--

\unrestrict 9QLPnaOUZQGBrqAluRBHwL4rgcOALKXDSiz6wwobndVcvgJizOCxAMmDU1rzHbj

