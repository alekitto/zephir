--
-- PostgreSQL database dump
--

-- Dumped from database version 12.2 (Debian 12.2-2.pgdg100+1)
-- Dumped by pg_dump version 12.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: zephir; Type: DATABASE; Schema: -; Owner: -
--

CREATE DATABASE zephir WITH TEMPLATE = template0 ENCODING = 'UTF8' LC_COLLATE = 'en_US.utf8' LC_CTYPE = 'en_US.utf8';


\connect zephir

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
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
-- Name: group; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public."group" (
    id character varying(1024) NOT NULL,
    policy_id character varying(1024)
);


--
-- Name: group_identity; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.group_identity (
    group_id character varying(1024) NOT NULL,
    identity_id character varying(1024) NOT NULL
);


--
-- Name: group_policy; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.group_policy (
    group_id character varying(1024) NOT NULL,
    policy_id character varying(1024) NOT NULL
);


--
-- Name: identity; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.identity (
    id character varying(1024) NOT NULL,
    policy_id character varying(1024)
);


--
-- Name: identity_policy; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.identity_policy (
    identity_id character varying(1024) NOT NULL,
    policy_id character varying(1024) NOT NULL
);


--
-- Name: policy; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.policy (
    id character varying(1024) NOT NULL,
    version integer DEFAULT 1 NOT NULL,
    effect boolean NOT NULL,
    actions jsonb NOT NULL,
    resources jsonb NOT NULL
);


--
-- Name: group_identity group_identity_pk; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.group_identity
    ADD CONSTRAINT group_identity_pk PRIMARY KEY (group_id, identity_id);


--
-- Name: group group_pk; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public."group"
    ADD CONSTRAINT group_pk PRIMARY KEY (id);


--
-- Name: group_policy group_policy_pk; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.group_policy
    ADD CONSTRAINT group_policy_pk PRIMARY KEY (group_id, policy_id);


--
-- Name: identity identity_pk; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.identity
    ADD CONSTRAINT identity_pk PRIMARY KEY (id);


--
-- Name: identity_policy identity_policy_pk; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.identity_policy
    ADD CONSTRAINT identity_policy_pk PRIMARY KEY (identity_id, policy_id);


--
-- Name: policy policy_pk; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.policy
    ADD CONSTRAINT policy_pk PRIMARY KEY (id);


--
-- Name: group_identity group_identity_group_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.group_identity
    ADD CONSTRAINT group_identity_group_id_fk FOREIGN KEY (group_id) REFERENCES public."group"(id) ON UPDATE RESTRICT ON DELETE CASCADE;


--
-- Name: group_identity group_identity_identity_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.group_identity
    ADD CONSTRAINT group_identity_identity_id_fk FOREIGN KEY (identity_id) REFERENCES public.identity(id) ON UPDATE RESTRICT ON DELETE CASCADE;


--
-- Name: group_policy group_policy_group_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.group_policy
    ADD CONSTRAINT group_policy_group_id_fk FOREIGN KEY (group_id) REFERENCES public."group"(id) ON UPDATE RESTRICT ON DELETE CASCADE;


--
-- Name: group group_policy_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public."group"
    ADD CONSTRAINT group_policy_id_fk FOREIGN KEY (policy_id) REFERENCES public.policy(id) ON UPDATE RESTRICT ON DELETE RESTRICT;


--
-- Name: group_policy group_policy_policy_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.group_policy
    ADD CONSTRAINT group_policy_policy_id_fk FOREIGN KEY (policy_id) REFERENCES public.policy(id) ON UPDATE RESTRICT ON DELETE CASCADE;


--
-- Name: identity identity_policy_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.identity
    ADD CONSTRAINT identity_policy_id_fk FOREIGN KEY (policy_id) REFERENCES public.policy(id) ON UPDATE RESTRICT ON DELETE RESTRICT;


--
-- Name: identity_policy identity_policy_identity_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.identity_policy
    ADD CONSTRAINT identity_policy_identity_id_fk FOREIGN KEY (identity_id) REFERENCES public.identity(id) ON UPDATE RESTRICT ON DELETE CASCADE;


--
-- Name: identity_policy identity_policy_policy_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.identity_policy
    ADD CONSTRAINT identity_policy_policy_id_fk FOREIGN KEY (policy_id) REFERENCES public.policy(id) ON UPDATE RESTRICT ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

