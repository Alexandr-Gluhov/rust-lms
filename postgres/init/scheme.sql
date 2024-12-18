--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2 (Debian 17.2-1.pgdg120+1)
-- Dumped by pg_dump version 17.0

-- Started on 2024-12-10 05:15:12 UTC

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
-- TOC entry 218 (class 1259 OID 16408)
-- Name: groups; Type: TABLE; Schema: public; Owner: pguser
--

CREATE TABLE public.groups (
    id integer NOT NULL,
    name character varying(255) NOT NULL,
    institute_id integer DEFAULT 1 NOT NULL
);


ALTER TABLE public.groups OWNER TO pguser;

--
-- TOC entry 217 (class 1259 OID 16407)
-- Name: groups_id_seq; Type: SEQUENCE; Schema: public; Owner: pguser
--

CREATE SEQUENCE public.groups_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.groups_id_seq OWNER TO pguser;

--
-- TOC entry 3399 (class 0 OID 0)
-- Dependencies: 217
-- Name: groups_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: pguser
--

ALTER SEQUENCE public.groups_id_seq OWNED BY public.groups.id;


--
-- TOC entry 222 (class 1259 OID 16436)
-- Name: institutes; Type: TABLE; Schema: public; Owner: pguser
--

CREATE TABLE public.institutes (
    id integer NOT NULL,
    name character varying(255)
);


ALTER TABLE public.institutes OWNER TO pguser;

--
-- TOC entry 221 (class 1259 OID 16435)
-- Name: institutes_id_seq; Type: SEQUENCE; Schema: public; Owner: pguser
--

CREATE SEQUENCE public.institutes_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.institutes_id_seq OWNER TO pguser;

--
-- TOC entry 3400 (class 0 OID 0)
-- Dependencies: 221
-- Name: institutes_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: pguser
--

ALTER SEQUENCE public.institutes_id_seq OWNED BY public.institutes.id;


--
-- TOC entry 224 (class 1259 OID 16449)
-- Name: news; Type: TABLE; Schema: public; Owner: pguser
--

CREATE TABLE public.news (
    id integer NOT NULL,
    text text NOT NULL,
    institute_id integer NOT NULL
);


ALTER TABLE public.news OWNER TO pguser;

--
-- TOC entry 223 (class 1259 OID 16448)
-- Name: news_id_seq; Type: SEQUENCE; Schema: public; Owner: pguser
--

CREATE SEQUENCE public.news_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.news_id_seq OWNER TO pguser;

--
-- TOC entry 3401 (class 0 OID 0)
-- Dependencies: 223
-- Name: news_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: pguser
--

ALTER SEQUENCE public.news_id_seq OWNED BY public.news.id;


--
-- TOC entry 220 (class 1259 OID 16422)
-- Name: users; Type: TABLE; Schema: public; Owner: pguser
--

CREATE TABLE public.users (
    id integer NOT NULL,
    name character varying(255) NOT NULL,
    surname character varying(255) NOT NULL,
    patronymic character varying(255),
    email character varying(255) NOT NULL,
    password character varying(255) NOT NULL,
    group_id integer NOT NULL
);


ALTER TABLE public.users OWNER TO pguser;

--
-- TOC entry 219 (class 1259 OID 16421)
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: pguser
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.users_id_seq OWNER TO pguser;

--
-- TOC entry 3402 (class 0 OID 0)
-- Dependencies: 219
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: pguser
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- TOC entry 3225 (class 2604 OID 16411)
-- Name: groups id; Type: DEFAULT; Schema: public; Owner: pguser
--

ALTER TABLE ONLY public.groups ALTER COLUMN id SET DEFAULT nextval('public.groups_id_seq'::regclass);


--
-- TOC entry 3228 (class 2604 OID 16439)
-- Name: institutes id; Type: DEFAULT; Schema: public; Owner: pguser
--

ALTER TABLE ONLY public.institutes ALTER COLUMN id SET DEFAULT nextval('public.institutes_id_seq'::regclass);


--
-- TOC entry 3229 (class 2604 OID 16452)
-- Name: news id; Type: DEFAULT; Schema: public; Owner: pguser
--

ALTER TABLE ONLY public.news ALTER COLUMN id SET DEFAULT nextval('public.news_id_seq'::regclass);


--
-- TOC entry 3227 (class 2604 OID 16425)
-- Name: users id; Type: DEFAULT; Schema: public; Owner: pguser
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- TOC entry 3387 (class 0 OID 16408)
-- Dependencies: 218
-- Data for Name: groups; Type: TABLE DATA; Schema: public; Owner: pguser
--

COPY public.groups (id, name, institute_id) FROM stdin;
1	ПРИб-222	1
2	ПРИб-221	1
\.


--
-- TOC entry 3391 (class 0 OID 16436)
-- Dependencies: 222
-- Data for Name: institutes; Type: TABLE DATA; Schema: public; Owner: pguser
--

COPY public.institutes (id, name) FROM stdin;
1	Институт математики и информационных технологий
\.


--
-- TOC entry 3393 (class 0 OID 16449)
-- Dependencies: 224
-- Data for Name: news; Type: TABLE DATA; Schema: public; Owner: pguser
--

COPY public.news (id, text, institute_id) FROM stdin;
1	Началась зимняя сессия	1
\.


--
-- TOC entry 3389 (class 0 OID 16422)
-- Dependencies: 220
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: pguser
--

COPY public.users (id, name, surname, patronymic, email, password, group_id) FROM stdin;
1	Иван	Иванов	Иванович	example@mail.com	sadfjalsdfj;	1
3	Александр	Глухов	Владимирович	prib-222_897175@volru.ru	$argon2id$v=19$m=19456,t=2,p=1$vwfxH4TeH5+LEJZuLvvTTA$cTpTFf1QkXk6FCqjAo873rs/hQuXhOzCavbhS8dbUhY	1
4	Александр	Глухов	Владимирович	sanjok250204@gmail.com	$argon2id$v=19$m=19456,t=2,p=1$nELlEYEnYsjiY+sJcY752g$nSFg7UGty8kL6UITZ83PE0zfopBjFZYfRQXkREi4Tfw	1
\.


--
-- TOC entry 3403 (class 0 OID 0)
-- Dependencies: 217
-- Name: groups_id_seq; Type: SEQUENCE SET; Schema: public; Owner: pguser
--

SELECT pg_catalog.setval('public.groups_id_seq', 2, true);


--
-- TOC entry 3404 (class 0 OID 0)
-- Dependencies: 221
-- Name: institutes_id_seq; Type: SEQUENCE SET; Schema: public; Owner: pguser
--

SELECT pg_catalog.setval('public.institutes_id_seq', 1, true);


--
-- TOC entry 3405 (class 0 OID 0)
-- Dependencies: 223
-- Name: news_id_seq; Type: SEQUENCE SET; Schema: public; Owner: pguser
--

SELECT pg_catalog.setval('public.news_id_seq', 1, true);


--
-- TOC entry 3406 (class 0 OID 0)
-- Dependencies: 219
-- Name: users_id_seq; Type: SEQUENCE SET; Schema: public; Owner: pguser
--

SELECT pg_catalog.setval('public.users_id_seq', 4, true);


--
-- TOC entry 3231 (class 2606 OID 16415)
-- Name: groups groups_pkey; Type: CONSTRAINT; Schema: public; Owner: pguser
--

ALTER TABLE ONLY public.groups
    ADD CONSTRAINT groups_pkey PRIMARY KEY (id);


--
-- TOC entry 3235 (class 2606 OID 16441)
-- Name: institutes institutes_pkey; Type: CONSTRAINT; Schema: public; Owner: pguser
--

ALTER TABLE ONLY public.institutes
    ADD CONSTRAINT institutes_pkey PRIMARY KEY (id);


--
-- TOC entry 3237 (class 2606 OID 16456)
-- Name: news news_pkey; Type: CONSTRAINT; Schema: public; Owner: pguser
--

ALTER TABLE ONLY public.news
    ADD CONSTRAINT news_pkey PRIMARY KEY (id);


--
-- TOC entry 3233 (class 2606 OID 16429)
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: pguser
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- TOC entry 3238 (class 2606 OID 16443)
-- Name: groups groups_institute_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: pguser
--

ALTER TABLE ONLY public.groups
    ADD CONSTRAINT groups_institute_id_fkey FOREIGN KEY (institute_id) REFERENCES public.institutes(id) NOT VALID;


--
-- TOC entry 3240 (class 2606 OID 16457)
-- Name: news news_institute_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: pguser
--

ALTER TABLE ONLY public.news
    ADD CONSTRAINT news_institute_id_fkey FOREIGN KEY (institute_id) REFERENCES public.institutes(id);


--
-- TOC entry 3239 (class 2606 OID 16430)
-- Name: users users_group_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: pguser
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_group_id_fkey FOREIGN KEY (group_id) REFERENCES public.groups(id) NOT VALID;


-- Completed on 2024-12-10 05:15:12 UTC

--
-- PostgreSQL database dump complete
--

