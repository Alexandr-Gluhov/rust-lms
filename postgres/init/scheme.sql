PGDMP  $            	        |            lms    17.2 (Debian 17.2-1.pgdg120+1)    17.0 #    D           0    0    ENCODING    ENCODING        SET client_encoding = 'UTF8';
                           false            E           0    0 
   STDSTRINGS 
   STDSTRINGS     (   SET standard_conforming_strings = 'on';
                           false            F           0    0 
   SEARCHPATH 
   SEARCHPATH     8   SELECT pg_catalog.set_config('search_path', '', false);
                           false            G           1262    16384    lms    DATABASE     n   CREATE DATABASE lms WITH TEMPLATE = template0 ENCODING = 'UTF8' LOCALE_PROVIDER = libc LOCALE = 'en_US.utf8';
    DROP DATABASE lms;
                     pguser    false            �            1259    16408    groups    TABLE     �   CREATE TABLE public.groups (
    id integer NOT NULL,
    name character varying(255) NOT NULL,
    institute_id integer DEFAULT 1 NOT NULL
);
    DROP TABLE public.groups;
       public         heap r       pguser    false            �            1259    16407    groups_id_seq    SEQUENCE     �   CREATE SEQUENCE public.groups_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 $   DROP SEQUENCE public.groups_id_seq;
       public               pguser    false    218            H           0    0    groups_id_seq    SEQUENCE OWNED BY     ?   ALTER SEQUENCE public.groups_id_seq OWNED BY public.groups.id;
          public               pguser    false    217            �            1259    16436 
   institutes    TABLE     ]   CREATE TABLE public.institutes (
    id integer NOT NULL,
    name character varying(255)
);
    DROP TABLE public.institutes;
       public         heap r       pguser    false            �            1259    16435    institutes_id_seq    SEQUENCE     �   CREATE SEQUENCE public.institutes_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 (   DROP SEQUENCE public.institutes_id_seq;
       public               pguser    false    222            I           0    0    institutes_id_seq    SEQUENCE OWNED BY     G   ALTER SEQUENCE public.institutes_id_seq OWNED BY public.institutes.id;
          public               pguser    false    221            �            1259    16449    news    TABLE     q   CREATE TABLE public.news (
    id integer NOT NULL,
    text text NOT NULL,
    institute_id integer NOT NULL
);
    DROP TABLE public.news;
       public         heap r       pguser    false            �            1259    16448    news_id_seq    SEQUENCE     �   CREATE SEQUENCE public.news_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 "   DROP SEQUENCE public.news_id_seq;
       public               pguser    false    224            J           0    0    news_id_seq    SEQUENCE OWNED BY     ;   ALTER SEQUENCE public.news_id_seq OWNED BY public.news.id;
          public               pguser    false    223            �            1259    16422    users    TABLE     -  CREATE TABLE public.users (
    id integer NOT NULL,
    name character varying(255) NOT NULL,
    surname character varying(255) NOT NULL,
    patronymic character varying(255),
    email character varying(255) NOT NULL,
    password character varying(255) NOT NULL,
    group_id integer NOT NULL
);
    DROP TABLE public.users;
       public         heap r       pguser    false            �            1259    16421    users_id_seq    SEQUENCE     �   CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
 #   DROP SEQUENCE public.users_id_seq;
       public               pguser    false    220            K           0    0    users_id_seq    SEQUENCE OWNED BY     =   ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;
          public               pguser    false    219            �           2604    16411 	   groups id    DEFAULT     f   ALTER TABLE ONLY public.groups ALTER COLUMN id SET DEFAULT nextval('public.groups_id_seq'::regclass);
 8   ALTER TABLE public.groups ALTER COLUMN id DROP DEFAULT;
       public               pguser    false    217    218    218            �           2604    16439    institutes id    DEFAULT     n   ALTER TABLE ONLY public.institutes ALTER COLUMN id SET DEFAULT nextval('public.institutes_id_seq'::regclass);
 <   ALTER TABLE public.institutes ALTER COLUMN id DROP DEFAULT;
       public               pguser    false    222    221    222            �           2604    16452    news id    DEFAULT     b   ALTER TABLE ONLY public.news ALTER COLUMN id SET DEFAULT nextval('public.news_id_seq'::regclass);
 6   ALTER TABLE public.news ALTER COLUMN id DROP DEFAULT;
       public               pguser    false    223    224    224            �           2604    16425    users id    DEFAULT     d   ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);
 7   ALTER TABLE public.users ALTER COLUMN id DROP DEFAULT;
       public               pguser    false    220    219    220            ;          0    16408    groups 
   TABLE DATA           8   COPY public.groups (id, name, institute_id) FROM stdin;
    public               pguser    false    218   K%       ?          0    16436 
   institutes 
   TABLE DATA           .   COPY public.institutes (id, name) FROM stdin;
    public               pguser    false    222   �%       A          0    16449    news 
   TABLE DATA           6   COPY public.news (id, text, institute_id) FROM stdin;
    public               pguser    false    224   �%       =          0    16422    users 
   TABLE DATA           Y   COPY public.users (id, name, surname, patronymic, email, password, group_id) FROM stdin;
    public               pguser    false    220   3&       L           0    0    groups_id_seq    SEQUENCE SET     ;   SELECT pg_catalog.setval('public.groups_id_seq', 2, true);
          public               pguser    false    217            M           0    0    institutes_id_seq    SEQUENCE SET     ?   SELECT pg_catalog.setval('public.institutes_id_seq', 1, true);
          public               pguser    false    221            N           0    0    news_id_seq    SEQUENCE SET     9   SELECT pg_catalog.setval('public.news_id_seq', 1, true);
          public               pguser    false    223            O           0    0    users_id_seq    SEQUENCE SET     :   SELECT pg_catalog.setval('public.users_id_seq', 4, true);
          public               pguser    false    219            �           2606    16415    groups groups_pkey 
   CONSTRAINT     P   ALTER TABLE ONLY public.groups
    ADD CONSTRAINT groups_pkey PRIMARY KEY (id);
 <   ALTER TABLE ONLY public.groups DROP CONSTRAINT groups_pkey;
       public                 pguser    false    218            �           2606    16441    institutes institutes_pkey 
   CONSTRAINT     X   ALTER TABLE ONLY public.institutes
    ADD CONSTRAINT institutes_pkey PRIMARY KEY (id);
 D   ALTER TABLE ONLY public.institutes DROP CONSTRAINT institutes_pkey;
       public                 pguser    false    222            �           2606    16456    news news_pkey 
   CONSTRAINT     L   ALTER TABLE ONLY public.news
    ADD CONSTRAINT news_pkey PRIMARY KEY (id);
 8   ALTER TABLE ONLY public.news DROP CONSTRAINT news_pkey;
       public                 pguser    false    224            �           2606    16429    users users_pkey 
   CONSTRAINT     N   ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);
 :   ALTER TABLE ONLY public.users DROP CONSTRAINT users_pkey;
       public                 pguser    false    220            �           2606    16443    groups groups_institute_id_fkey    FK CONSTRAINT     �   ALTER TABLE ONLY public.groups
    ADD CONSTRAINT groups_institute_id_fkey FOREIGN KEY (institute_id) REFERENCES public.institutes(id) NOT VALID;
 I   ALTER TABLE ONLY public.groups DROP CONSTRAINT groups_institute_id_fkey;
       public               pguser    false    222    3235    218            �           2606    16457    news news_institute_id_fkey    FK CONSTRAINT     �   ALTER TABLE ONLY public.news
    ADD CONSTRAINT news_institute_id_fkey FOREIGN KEY (institute_id) REFERENCES public.institutes(id);
 E   ALTER TABLE ONLY public.news DROP CONSTRAINT news_institute_id_fkey;
       public               pguser    false    3235    222    224            �           2606    16430    users users_group_id_fkey    FK CONSTRAINT     �   ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_group_id_fkey FOREIGN KEY (group_id) REFERENCES public.groups(id) NOT VALID;
 C   ALTER TABLE ONLY public.users DROP CONSTRAINT users_group_id_fkey;
       public               pguser    false    218    220    3231            ;   %   x�3�0�3.l�522�4�2B0
��qqq WL�      ?   W   x�3�0��ދ��.��t��b�=6 �[���.�P #��.6�eڀ�}�ź/�*�4\lr�]�ě��;�b���� �]@r      A   <   x�3�0���6\��/�(\�~aǅ=�^�دp���V�h��9�b���� rZ1      =   A  x���]K�`ǯ�}��i�77	A�-�����$�����67�+#�ۨ�>���d�ξQ�@�*���9�9?~�l`�>a�k�]�����i�UM�0�]��R�(��P%���WxKo���
�3�ӻ�����k���3��Bst�1�*<�s��w¸ƈVC���9��*S��,X���Uq>�2t2�M�L�\N[�XNB���DҘ�=��R�ڪ�_
�c�����&#�'LF}Cɼ��{G�g�6抸����c���D��S"�TrQk���i�'�|�lv#�r�N�B�B,.5?8����u;�+�,���e���/ ��2     