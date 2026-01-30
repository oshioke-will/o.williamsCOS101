--
-- PostgreSQL database dump
--

\restrict bCLkLs014eXCyuFrubT9A3P7vkw17sKyYriacj6GjvgFodA7b3vZ89bIokH81OF

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
-- Name: dataplans; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplans (
    data_id integer NOT NULL,
    data_size character varying(20),
    data_duration_days integer,
    data_price_naira integer
);


ALTER TABLE public.dataplans OWNER TO postgres;

--
-- Data for Name: dataplans; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplans (data_id, data_size, data_duration_days, data_price_naira) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.8GB	30	2500
7	14GB	30	3000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


--
-- Name: dataplans dataplans_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplans
    ADD CONSTRAINT dataplans_pkey PRIMARY KEY (data_id);


--
-- PostgreSQL database dump complete
--

\unrestrict bCLkLs014eXCyuFrubT9A3P7vkw17sKyYriacj6GjvgFodA7b3vZ89bIokH81OF

