<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create CartDiscount</name>
   <tag></tag>
   <elementGuidId>fd0dbad9-b850-4460-a4bd-e21e8a7d2479</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot; : {\n    \&quot;en\&quot; : \&quot;Summer Sale\&quot;\n  },\n  \&quot;value\&quot; : {\n    \&quot;type\&quot; : \&quot;relative\&quot;,\n    \&quot;permyriad\&quot; : 1000\n  },\n  \&quot;cartPredicate\&quot; : \&quot;1\u003d1\&quot;,\n  \&quot;target\&quot; : {\n    \&quot;type\&quot; : \&quot;lineItems\&quot;,\n    \&quot;predicate\&quot; : \&quot;1\u003d1\&quot;\n  },\n  \&quot;sortOrder\&quot; : \&quot;0.1\&quot;,\n  \&quot;stores\&quot; : [ {\n    \&quot;key\&quot; : \&quot;${store-key}\&quot;\n  } ],\n  \&quot;isActive\&quot; : true,\n  \&quot;requiresDiscountCode\&quot; : false\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer {{ctp_access_token}}</value>
      <webElementGuid>bcb536a0-3f0e-46da-9f47-90f8a129b5ea</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>75509730-5283-43d6-a54e-216b80e43352</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/${project-key}/in-store/key=${store-key}/cart-discounts</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>eecf5c10-5386-4c79-8d2a-2d974d4b832c</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project-key</defaultValue>
      <description></description>
      <id>0c66db95-a748-44a0-8a44-95e606604df7</id>
      <masked>false</masked>
      <name>project-key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.store-key</defaultValue>
      <description></description>
      <id>f0b4510e-2724-43de-92ce-bcbbfe0879ff</id>
      <masked>false</masked>
      <name>store-key</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
